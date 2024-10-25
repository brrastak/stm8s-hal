#[doc = "Register `TDRL` reader"]
pub type R = crate::R<TDRL_SPEC>;
#[doc = "Register `TDRL` writer"]
pub type W = crate::W<TDRL_SPEC>;
#[doc = "Field `TL` reader - "]
pub type TL_R = crate::FieldReader;
#[doc = "Field `TL` writer - "]
pub type TL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tl(&self) -> TL_R {
        TL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn tl(&mut self) -> TL_W<TDRL_SPEC> {
        TL_W::new(self, 0)
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
#[doc = "ADC Schmitt trigger disable register low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TDRL_SPEC;
impl crate::RegisterSpec for TDRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tdrl::R`](R) reader structure"]
impl crate::Readable for TDRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tdrl::W`](W) writer structure"]
impl crate::Writable for TDRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDRL to value 0"]
impl crate::Resettable for TDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
