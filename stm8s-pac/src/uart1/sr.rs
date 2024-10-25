#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `PE` reader - "]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - "]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - "]
pub type FE_R = crate::BitReader;
#[doc = "Field `FE` writer - "]
pub type FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NF` reader - "]
pub type NF_R = crate::BitReader;
#[doc = "Field `NF` writer - "]
pub type NF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OR_LHE` reader - "]
pub type OR_LHE_R = crate::BitReader;
#[doc = "Field `OR_LHE` writer - "]
pub type OR_LHE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE` reader - "]
pub type IDLE_R = crate::BitReader;
#[doc = "Field `IDLE` writer - "]
pub type IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - "]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `RXNE` writer - "]
pub type RXNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - "]
pub type TC_R = crate::BitReader;
#[doc = "Field `TC` writer - "]
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - "]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - "]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn or_lhe(&self) -> OR_LHE_R {
        OR_LHE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<SR_SPEC> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<SR_SPEC> {
        FE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nf(&mut self) -> NF_W<SR_SPEC> {
        NF_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn or_lhe(&mut self) -> OR_LHE_W<SR_SPEC> {
        OR_LHE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<SR_SPEC> {
        IDLE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<SR_SPEC> {
        RXNE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<SR_SPEC> {
        TC_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<SR_SPEC> {
        TXE_W::new(self, 7)
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
#[doc = "UART1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0xc0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
