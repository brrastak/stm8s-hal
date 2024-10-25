#[doc = "Register `PECR` reader"]
pub type R = crate::R<PECR_SPEC>;
#[doc = "Register `PECR` writer"]
pub type W = crate::W<PECR_SPEC>;
#[doc = "Field `PEC` reader - "]
pub type PEC_R = crate::FieldReader;
#[doc = "Field `PEC` writer - "]
pub type PEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<PECR_SPEC> {
        PEC_W::new(self, 0)
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
#[doc = "I2C packet error checking register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pecr::R`](R) reader structure"]
impl crate::Readable for PECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pecr::W`](W) writer structure"]
impl crate::Writable for PECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PECR to value 0"]
impl crate::Resettable for PECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
