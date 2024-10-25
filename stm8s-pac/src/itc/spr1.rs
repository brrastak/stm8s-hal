#[doc = "Register `SPR1` reader"]
pub type R = crate::R<SPR1_SPEC>;
#[doc = "Register `SPR1` writer"]
pub type W = crate::W<SPR1_SPEC>;
#[doc = "Field `VECT0SPR` reader - "]
pub type VECT0SPR_R = crate::FieldReader;
#[doc = "Field `VECT0SPR` writer - "]
pub type VECT0SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT1SPR` reader - "]
pub type VECT1SPR_R = crate::FieldReader;
#[doc = "Field `VECT1SPR` writer - "]
pub type VECT1SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT2SPR` reader - "]
pub type VECT2SPR_R = crate::FieldReader;
#[doc = "Field `VECT2SPR` writer - "]
pub type VECT2SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VECT3SPR` reader - "]
pub type VECT3SPR_R = crate::FieldReader;
#[doc = "Field `VECT3SPR` writer - "]
pub type VECT3SPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vect0spr(&self) -> VECT0SPR_R {
        VECT0SPR_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vect1spr(&self) -> VECT1SPR_R {
        VECT1SPR_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn vect2spr(&self) -> VECT2SPR_R {
        VECT2SPR_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn vect3spr(&self) -> VECT3SPR_R {
        VECT3SPR_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vect0spr(&mut self) -> VECT0SPR_W<SPR1_SPEC> {
        VECT0SPR_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn vect1spr(&mut self) -> VECT1SPR_W<SPR1_SPEC> {
        VECT1SPR_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn vect2spr(&mut self) -> VECT2SPR_W<SPR1_SPEC> {
        VECT2SPR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn vect3spr(&mut self) -> VECT3SPR_W<SPR1_SPEC> {
        VECT3SPR_W::new(self, 6)
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
#[doc = "Interrupt software priority register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPR1_SPEC;
impl crate::RegisterSpec for SPR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spr1::R`](R) reader structure"]
impl crate::Readable for SPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spr1::W`](W) writer structure"]
impl crate::Writable for SPR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPR1 to value 0xff"]
impl crate::Resettable for SPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
