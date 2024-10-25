#[doc = "Register `PCKENR2` reader"]
pub type R = crate::R<PCKENR2_SPEC>;
#[doc = "Register `PCKENR2` writer"]
pub type W = crate::W<PCKENR2_SPEC>;
#[doc = "Field `PCKEN2` reader - "]
pub type PCKEN2_R = crate::FieldReader;
#[doc = "Field `PCKEN2` writer - "]
pub type PCKEN2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pcken2(&self) -> PCKEN2_R {
        PCKEN2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pcken2(&mut self) -> PCKEN2_W<PCKENR2_SPEC> {
        PCKEN2_W::new(self, 0)
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
#[doc = "Peripheral clock gating register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pckenr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckenr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCKENR2_SPEC;
impl crate::RegisterSpec for PCKENR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pckenr2::R`](R) reader structure"]
impl crate::Readable for PCKENR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pckenr2::W`](W) writer structure"]
impl crate::Writable for PCKENR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCKENR2 to value 0xff"]
impl crate::Resettable for PCKENR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
