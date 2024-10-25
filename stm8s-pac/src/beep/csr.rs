#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `BEEPDIV` reader - "]
pub type BEEPDIV_R = crate::FieldReader;
#[doc = "Field `BEEPDIV` writer - "]
pub type BEEPDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BEEPEN` reader - "]
pub type BEEPEN_R = crate::BitReader;
#[doc = "Field `BEEPEN` writer - "]
pub type BEEPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEEPSEL` reader - "]
pub type BEEPSEL_R = crate::FieldReader;
#[doc = "Field `BEEPSEL` writer - "]
pub type BEEPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn beepdiv(&self) -> BEEPDIV_R {
        BEEPDIV_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn beepen(&self) -> BEEPEN_R {
        BEEPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn beepsel(&self) -> BEEPSEL_R {
        BEEPSEL_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn beepdiv(&mut self) -> BEEPDIV_W<CSR_SPEC> {
        BEEPDIV_W::new(self, 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn beepen(&mut self) -> BEEPEN_W<CSR_SPEC> {
        BEEPEN_W::new(self, 5)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn beepsel(&mut self) -> BEEPSEL_W<CSR_SPEC> {
        BEEPSEL_W::new(self, 6)
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
#[doc = "BEEP control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x1f"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
