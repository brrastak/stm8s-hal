#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `RXNE` reader - "]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `RXNE` writer - "]
pub type RXNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - "]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - "]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP` reader - "]
pub type WKUP_R = crate::BitReader;
#[doc = "Field `WKUP` writer - "]
pub type WKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCERR` reader - "]
pub type CRCERR_R = crate::BitReader;
#[doc = "Field `CRCERR` writer - "]
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` reader - "]
pub type MODF_R = crate::BitReader;
#[doc = "Field `MODF` writer - "]
pub type MODF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - "]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - "]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSY` reader - "]
pub type BSY_R = crate::BitReader;
#[doc = "Field `BSY` writer - "]
pub type BSY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<SR_SPEC> {
        RXNE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<SR_SPEC> {
        TXE_W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<SR_SPEC> {
        WKUP_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<SR_SPEC> {
        CRCERR_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn modf(&mut self) -> MODF_W<SR_SPEC> {
        MODF_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<SR_SPEC> {
        OVR_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bsy(&mut self) -> BSY_W<SR_SPEC> {
        BSY_W::new(self, 7)
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
#[doc = "SPI status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
