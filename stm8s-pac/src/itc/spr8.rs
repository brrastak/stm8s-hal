#[doc = "Register `SPR8` reader"]
pub type R = crate::R<SPR8_SPEC>;
#[doc = "Register `SPR8` writer"]
pub type W = crate::W<SPR8_SPEC>;
#[doc = "Field `VECT28SPR` reader - "]
pub type VECT28SPR_R = crate::FieldReader;
#[doc = "Field `VECT28SPR` writer - "]
pub type VECT28SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT29SPR` reader - "]
pub type VECT29SPR_R = crate::FieldReader;
#[doc = "Field `VECT29SPR` writer - "]
pub type VECT29SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect28spr(&self) -> VECT28SPR_R {
        VECT28SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect29spr(&self) -> VECT29SPR_R {
        VECT29SPR_R::new((self.bits >> 2) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect28spr(&mut self) -> VECT28SPR_W<SPR8_SPEC> {
        VECT28SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect29spr(&mut self) -> VECT29SPR_W<SPR8_SPEC> {
        VECT29SPR_W::new(self, 2)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt software priority register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR8_SPEC;
impl crate::RegisterSpec for SPR8_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr8::R`](R) reader structure"]
impl crate::Readable for SPR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr8::W`](W) writer structure"]
impl crate::Writable for SPR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR8 to value 0xff"]
impl crate::Resettable for SPR8_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
