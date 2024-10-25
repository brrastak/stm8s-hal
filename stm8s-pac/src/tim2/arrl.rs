#[doc = "Register `ARRL` reader"]
pub type R = crate::R<ARRL_SPEC>;
#[doc = "Register `ARRL` writer"]
pub type W = crate::W<ARRL_SPEC>;
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
    pub fn arr(&mut self) -> ARR_W<ARRL_SPEC> {
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
#[doc = "TIM2 auto-reload register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARRL_SPEC;
impl crate::RegisterSpec for ARRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`arrl::R`](R) reader structure"]
impl crate::Readable for ARRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arrl::W`](W) writer structure"]
impl crate::Writable for ARRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARRL to value 0xff"]
impl crate::Resettable for ARRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
