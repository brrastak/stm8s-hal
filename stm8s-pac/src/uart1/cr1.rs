#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `PIEN` reader - "]
pub type PIEN_R = crate::BitReader;
#[doc = "Field `PIEN` writer - "]
pub type PIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - "]
pub type PS_R = crate::BitReader;
#[doc = "Field `PS` writer - "]
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCEN` reader - "]
pub type PCEN_R = crate::BitReader;
#[doc = "Field `PCEN` writer - "]
pub type PCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - "]
pub type WAKE_R = crate::BitReader;
#[doc = "Field `WAKE` writer - "]
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - "]
pub type M_R = crate::BitReader;
#[doc = "Field `M` writer - "]
pub type M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0` reader - "]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART0` writer - "]
pub type UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T8` reader - "]
pub type T8_R = crate::BitReader;
#[doc = "Field `T8` writer - "]
pub type T8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R8` reader - "]
pub type R8_R = crate::BitReader;
#[doc = "Field `R8` writer - "]
pub type R8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pien(&self) -> PIEN_R {
        PIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t8(&self) -> T8_R {
        T8_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn r8(&self) -> R8_R {
        R8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pien(&mut self) -> PIEN_W<CR1_SPEC> {
        PIEN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<CR1_SPEC> {
        PS_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcen(&mut self) -> PCEN_W<CR1_SPEC> {
        PCEN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<CR1_SPEC> {
        WAKE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<CR1_SPEC> {
        M_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<CR1_SPEC> {
        UART0_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn t8(&mut self) -> T8_W<CR1_SPEC> {
        T8_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn r8(&mut self) -> R8_W<CR1_SPEC> {
        R8_W::new(self, 7)
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
#[doc = "UART1 control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
