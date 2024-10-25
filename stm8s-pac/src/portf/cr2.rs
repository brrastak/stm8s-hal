#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2_SPEC>;
#[doc = "Field `C20` reader - "]
pub type C20_R = crate::BitReader;
#[doc = "Field `C20` writer - "]
pub type C20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C21` reader - "]
pub type C21_R = crate::BitReader;
#[doc = "Field `C21` writer - "]
pub type C21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C22` reader - "]
pub type C22_R = crate::BitReader;
#[doc = "Field `C22` writer - "]
pub type C22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C23` reader - "]
pub type C23_R = crate::BitReader;
#[doc = "Field `C23` writer - "]
pub type C23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C24` reader - "]
pub type C24_R = crate::BitReader;
#[doc = "Field `C24` writer - "]
pub type C24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C25` reader - "]
pub type C25_R = crate::BitReader;
#[doc = "Field `C25` writer - "]
pub type C25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C26` reader - "]
pub type C26_R = crate::BitReader;
#[doc = "Field `C26` writer - "]
pub type C26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C27` reader - "]
pub type C27_R = crate::BitReader;
#[doc = "Field `C27` writer - "]
pub type C27_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn c20(&self) -> C20_R {
        C20_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn c21(&self) -> C21_R {
        C21_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn c22(&self) -> C22_R {
        C22_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c23(&self) -> C23_R {
        C23_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn c24(&self) -> C24_R {
        C24_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn c25(&self) -> C25_R {
        C25_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn c26(&self) -> C26_R {
        C26_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn c27(&self) -> C27_R {
        C27_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn c20(&mut self) -> C20_W<CR2_SPEC> {
        C20_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn c21(&mut self) -> C21_W<CR2_SPEC> {
        C21_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn c22(&mut self) -> C22_W<CR2_SPEC> {
        C22_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn c23(&mut self) -> C23_W<CR2_SPEC> {
        C23_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn c24(&mut self) -> C24_W<CR2_SPEC> {
        C24_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn c25(&mut self) -> C25_W<CR2_SPEC> {
        C25_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn c26(&mut self) -> C26_W<CR2_SPEC> {
        C26_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn c27(&mut self) -> C27_W<CR2_SPEC> {
        C27_W::new(self, 7)
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
#[doc = "Port F control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
