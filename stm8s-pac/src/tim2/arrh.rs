#[doc = "Register `ARRH` reader"]
pub type R = crate::R<ARRH_SPEC>;
#[doc = "Register `ARRH` writer"]
pub type W = crate::W<ARRH_SPEC>;
#[doc = "Field `ARR` reader - "]
pub type ARR_R = crate::FieldReader;
#[doc = "Field `ARR` writer - "]
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<ARRH_SPEC> {
        ARR_W::new(self, 0)
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
#[doc = "TIM2 auto-reload register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARRH_SPEC;
impl crate::RegisterSpec for ARRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`arrh::R`](R) reader structure"]
impl crate::Readable for ARRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arrh::W`](W) writer structure"]
impl crate::Writable for ARRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARRH to value 0xff"]
impl crate::Resettable for ARRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
