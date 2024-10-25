#[doc = "Register `SR1` reader"]
pub type R = crate::R<SR1_SPEC>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<SR1_SPEC>;
#[doc = "Field `SB` reader - "]
pub type SB_R = crate::BitReader;
#[doc = "Field `SB` writer - "]
pub type SB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::BitReader;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTF` reader - "]
pub type BTF_R = crate::BitReader;
#[doc = "Field `BTF` writer - "]
pub type BTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD10` reader - "]
pub type ADD10_R = crate::BitReader;
#[doc = "Field `ADD10` writer - "]
pub type ADD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPF` reader - "]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `STOPF` writer - "]
pub type STOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNE` reader - "]
pub type RXNE_R = crate::BitReader;
#[doc = "Field `RXNE` writer - "]
pub type RXNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - "]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - "]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 6) & 1) != 0)
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
    pub fn sb(&mut self) -> SB_W<SR1_SPEC> {
        SB_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<SR1_SPEC> {
        ADDR_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn btf(&mut self) -> BTF_W<SR1_SPEC> {
        BTF_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<SR1_SPEC> {
        ADD10_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn stopf(&mut self) -> STOPF_W<SR1_SPEC> {
        STOPF_W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<SR1_SPEC> {
        RXNE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<SR1_SPEC> {
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
#[doc = "I2C status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for SR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for SR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
