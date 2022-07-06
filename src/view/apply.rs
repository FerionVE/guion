pub trait ApplyTo<T,E>: 'static where T: Sized {
    fn apply_to(self, apply_to: &mut T);
}

impl<'r,T,V,E> ApplyTo<&'r mut T,E> for V where V: ApplyTo<T,E> { //TODO the blanket impl may be too aggressive
    #[inline]
    fn apply_to(self, apply_to: &mut &'r mut T) {
        self.apply_to(*apply_to)
    }
}

#[macro_export]
macro_rules! mutor_apply {
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))?  => |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        $crate::mutor_apply!(
            $e;$mutor $(($($extra_out),*))? ?=> |$root,$ctx $(,$($extra_in),*)?| {match $root {
                $crate::error::ResolveResult::Ok($root) => {$mutexpr;},
                $crate::error::ResolveResult::Err(_) => {/*TODO*/},
            }}
        )
    };
    (
        $e:ty;$mutor:ident $(($($extra_out:expr),*))? ?=> |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            #[inline] move |$root,_,$ctx,__recieved_apply $(,$($extra_in),*)?| {
                ($mutor)(
                    $root,&(),
                    &mut move |$root,_,$ctx| {
                        $crate::view::applion::ApplyTo::<_,$e>::apply(__recieved_apply,$mutexpr);
                    },
                    $ctx $(,$($extra_out),*)?
                )
            }
        }
    };
    (
        $e:ty;$($mutor:ident)?                        |=> |$root:ident,$ctx:ident $(,$($extra_in:ident),*)?| $mutexpr:expr
    ) => {
        {
            $(let $mutor = $mutor.clone();)?
            #[inline] move |$root,_,$ctx,__recieved_apply $(,$($extra_in),*)?| {
                $crate::view::applion::ApplyTo::<_,$e>::apply(__recieved_apply,$mutexpr);
            }
        }
    };
}

pub(crate) use mutor_apply;
