#[doc = "Register `SPR6` reader"]
pub type R = crate::R<SPR6_SPEC>;
#[doc = "Register `SPR6` writer"]
pub type W = crate::W<SPR6_SPEC>;
#[doc = "Field `VECT20SPR` reader - "]
pub type VECT20SPR_R = crate::FieldReader;
#[doc = "Field `VECT20SPR` writer - "]
pub type VECT20SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT21SPR` reader - "]
pub type VECT21SPR_R = crate::FieldReader;
#[doc = "Field `VECT21SPR` writer - "]
pub type VECT21SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT22SPR` reader - "]
pub type VECT22SPR_R = crate::FieldReader;
#[doc = "Field `VECT22SPR` writer - "]
pub type VECT22SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT23SPR` reader - "]
pub type VECT23SPR_R = crate::FieldReader;
#[doc = "Field `VECT23SPR` writer - "]
pub type VECT23SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect20spr(&self) -> VECT20SPR_R {
        VECT20SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect21spr(&self) -> VECT21SPR_R {
        VECT21SPR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vect22spr(&self) -> VECT22SPR_R {
        VECT22SPR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn vect23spr(&self) -> VECT23SPR_R {
        VECT23SPR_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect20spr(&mut self) -> VECT20SPR_W<SPR6_SPEC> {
        VECT20SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect21spr(&mut self) -> VECT21SPR_W<SPR6_SPEC> {
        VECT21SPR_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn vect22spr(&mut self) -> VECT22SPR_W<SPR6_SPEC> {
        VECT22SPR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn vect23spr(&mut self) -> VECT23SPR_W<SPR6_SPEC> {
        VECT23SPR_W::new(self, 6)
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
#[doc = "Interrupt software priority register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR6_SPEC;
impl crate::RegisterSpec for SPR6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr6::R`](R) reader structure"]
impl crate::Readable for SPR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr6::W`](W) writer structure"]
impl crate::Writable for SPR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR6 to value 0xff"]
impl crate::Resettable for SPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
