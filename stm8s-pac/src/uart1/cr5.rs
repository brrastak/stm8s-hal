#[doc = "Register `CR5` reader"]
pub type R = crate::R<CR5_SPEC>;
#[doc = "Register `CR5` writer"]
pub type W = crate::W<CR5_SPEC>;
#[doc = "Field `IREN` reader - "]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - "]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRLP` reader - "]
pub type IRLP_R = crate::BitReader;
#[doc = "Field `IRLP` writer - "]
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - "]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `HDSEL` writer - "]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - "]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - "]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCEN` reader - "]
pub type SCEN_R = crate::BitReader;
#[doc = "Field `SCEN` writer - "]
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<CR5_SPEC> {
        IREN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<CR5_SPEC> {
        IRLP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<CR5_SPEC> {
        HDSEL_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<CR5_SPEC> {
        NACK_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<CR5_SPEC> {
        SCEN_W::new(self, 5)
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
#[doc = "UART1 control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR5_SPEC;
impl crate::RegisterSpec for CR5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr5::R`](R) reader structure"]
impl crate::Readable for CR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr5::W`](W) writer structure"]
impl crate::Writable for CR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR5 to value 0"]
impl crate::Resettable for CR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
