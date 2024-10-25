#[doc = "Register `PCKENR1` reader"]
pub type R = crate::R<PCKENR1_SPEC>;
#[doc = "Register `PCKENR1` writer"]
pub type W = crate::W<PCKENR1_SPEC>;
#[doc = "Field `PCKEN` reader - "]
pub type PCKEN_R = crate::FieldReader;
#[doc = "Field `PCKEN` writer - "]
pub type PCKEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pcken(&self) -> PCKEN_R {
        PCKEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn pcken(&mut self) -> PCKEN_W<PCKENR1_SPEC> {
        PCKEN_W::new(self, 0)
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
#[doc = "Peripheral clock gating register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pckenr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pckenr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCKENR1_SPEC;
impl crate::RegisterSpec for PCKENR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pckenr1::R`](R) reader structure"]
impl crate::Readable for PCKENR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pckenr1::W`](W) writer structure"]
impl crate::Writable for PCKENR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCKENR1 to value 0xff"]
impl crate::Resettable for PCKENR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
