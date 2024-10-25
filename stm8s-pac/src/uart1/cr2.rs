#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `SBK` reader - "]
pub type SBK_R = crate::BitReader;
#[doc = "Field `SBK` writer - "]
pub type SBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - "]
pub type RWU_R = crate::BitReader;
#[doc = "Field `RWU` writer - "]
pub type RWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - "]
pub type REN_R = crate::BitReader;
#[doc = "Field `REN` writer - "]
pub type REN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - "]
pub type TEN_R = crate::BitReader;
#[doc = "Field `TEN` writer - "]
pub type TEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIEN` reader - "]
pub type ILIEN_R = crate::BitReader;
#[doc = "Field `ILIEN` writer - "]
pub type ILIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIEN` reader - "]
pub type RIEN_R = crate::BitReader;
#[doc = "Field `RIEN` writer - "]
pub type RIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIEN` reader - "]
pub type TCIEN_R = crate::BitReader;
#[doc = "Field `TCIEN` writer - "]
pub type TCIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIEN` reader - "]
pub type TIEN_R = crate::BitReader;
#[doc = "Field `TIEN` writer - "]
pub type TIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ren(&self) -> REN_R {
        REN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ilien(&self) -> ILIEN_R {
        ILIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tien(&self) -> TIEN_R {
        TIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SBK_W<CR2_SPEC> {
        SBK_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<CR2_SPEC> {
        RWU_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> REN_W<CR2_SPEC> {
        REN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<CR2_SPEC> {
        TEN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ilien(&mut self) -> ILIEN_W<CR2_SPEC> {
        ILIEN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rien(&mut self) -> RIEN_W<CR2_SPEC> {
        RIEN_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TCIEN_W<CR2_SPEC> {
        TCIEN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tien(&mut self) -> TIEN_W<CR2_SPEC> {
        TIEN_W::new(self, 7)
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
#[doc = "UART1 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
