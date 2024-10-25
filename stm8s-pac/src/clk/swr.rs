#[doc = "Register `SWR` reader"]
pub type R = crate::R<SWR_SPEC>;
#[doc = "Register `SWR` writer"]
pub type W = crate::W<SWR_SPEC>;
#[doc = "Field `SWI` reader - "]
pub type SWI_R = crate::FieldReader;
#[doc = "Field `SWI` writer - "]
pub type SWI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn swi(&self) -> SWI_R {
        SWI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn swi(&mut self) -> SWI_W<SWR_SPEC> {
        SWI_W::new(self, 0)
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
#[doc = "Clock master switch register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWR_SPEC;
impl crate::RegisterSpec for SWR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`swr::R`](R) reader structure"]
impl crate::Readable for SWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swr::W`](W) writer structure"]
impl crate::Writable for SWR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWR to value 0xe1"]
impl crate::Resettable for SWR_SPEC {
    const RESET_VALUE: Self::Ux = 0xe1;
}
