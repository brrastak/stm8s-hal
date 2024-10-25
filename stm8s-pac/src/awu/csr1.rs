#[doc = "Register `CSR1` reader"]
pub type R = crate::R<CSR1_SPEC>;
#[doc = "Register `CSR1` writer"]
pub type W = crate::W<CSR1_SPEC>;
#[doc = "Field `MSR` reader - "]
pub type MSR_R = crate::BitReader;
#[doc = "Field `MSR` writer - "]
pub type MSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWUEN` reader - "]
pub type AWUEN_R = crate::BitReader;
#[doc = "Field `AWUEN` writer - "]
pub type AWUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWUF` reader - "]
pub type AWUF_R = crate::BitReader;
#[doc = "Field `AWUF` writer - "]
pub type AWUF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn msr(&self) -> MSR_R {
        MSR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn awuen(&self) -> AWUEN_R {
        AWUEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn awuf(&self) -> AWUF_R {
        AWUF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn msr(&mut self) -> MSR_W<CSR1_SPEC> {
        MSR_W::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn awuen(&mut self) -> AWUEN_W<CSR1_SPEC> {
        AWUEN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn awuf(&mut self) -> AWUF_W<CSR1_SPEC> {
        AWUF_W::new(self, 5)
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
#[doc = "AWU control/status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR1_SPEC;
impl crate::RegisterSpec for CSR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csr1::R`](R) reader structure"]
impl crate::Readable for CSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr1::W`](W) writer structure"]
impl crate::Writable for CSR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR1 to value 0"]
impl crate::Resettable for CSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
