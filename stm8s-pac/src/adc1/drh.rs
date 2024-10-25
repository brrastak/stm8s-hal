#[doc = "Register `DRH` reader"]
pub type R = crate::R<DRH_SPEC>;
#[doc = "Register `DRH` writer"]
pub type W = crate::W<DRH_SPEC>;
#[doc = "Field `DH` reader - "]
pub type DH_R = crate::FieldReader;
#[doc = "Field `DH` writer - "]
pub type DH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dh(&self) -> DH_R {
        DH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dh(&mut self) -> DH_W<DRH_SPEC> {
        DH_W::new(self, 0)
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
#[doc = "ADC data register high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRH_SPEC;
impl crate::RegisterSpec for DRH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`drh::R`](R) reader structure"]
impl crate::Readable for DRH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`drh::W`](W) writer structure"]
impl crate::Writable for DRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRH to value 0"]
impl crate::Resettable for DRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
