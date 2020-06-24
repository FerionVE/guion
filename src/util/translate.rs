pub use super::*;

pub struct Translate<'a>(&'a Bounds,&'a Bounds);

impl<'a> Translate<'a> {
    pub fn translate_pos<T>(&self, off: T) -> Offset where T: AsRef<Offset> {
        self.translate_i32(off.as_ref().clone().into()).into()
    }
    pub fn translate_i32(&self, p: (i32,i32)) -> (i32,i32) {
        (
            ((p.0 - self.0.off.x) * (self.1.size.w as i32)).div_or_nop(self.0.size.w as i32) + self.1.off.x,
            ((p.1 - self.0.off.y) * (self.1.size.h as i32)).div_or_nop(self.0.size.h as i32) + self.1.off.y,
        )
    }
    pub fn translate_u32(&self, p: (u32,u32)) -> (u32,u32) {
        let (x,y) = self.translate_i32((p.0 as i32,p.1 as i32));
        (x as u32,y as u32)
    }
    pub fn translate_f32(&self, p: (f32,f32)) -> (f32,f32) {
        (
            (p.0 - self.0.off.x as f32) * (self.1.size.w as f32) / (self.0.size.w.max(1) as f32) + (self.1.off.x as f32),
            (p.1 - self.0.off.y as f32) * (self.1.size.h as f32) / (self.0.size.h.max(1) as f32) + (self.1.off.y as f32),
        )
    }
    
    pub fn reverse(&self) -> Translate<'a> {
        Translate(self.1,self.0)
    }
}
