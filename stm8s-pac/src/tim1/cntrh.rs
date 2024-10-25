#[doc = "Register `CNTRH` reader"]
pub type R = crate::R<CNTRH_SPEC>;
#[doc = "Register `CNTRH` writer"]
pub type W = crate::W<CNTRH_SPEC>;
#[doc = "Field `CNT` reader - "]
pub type CNT_R = crate::FieldReader;
#[doc = "Field `CNT` writer - "]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CNTRH_SPEC> {
        CNT_W::new(self, 0)
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
#[doc = "TIM1 counter high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTRH_SPEC;
impl crate::RegisterSpec for CNTRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cntrh::R`](R) reader structure"]
impl crate::Readable for CNTRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntrh::W`](W) writer structure"]
impl crate::Writable for CNTRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTRH to value 0"]
impl crate::Resettable for CNTRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
