use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12.765 21.522a.5.5 0 0 1-.765-.424v-8.196a.5.5 0 0 1 .765-.424l5.878 3.674a1 1 0 0 1 0 1.696z" ></ path > < path d = "M14.12 3.88 16 2" ></ path > < path d = "M18 11a4 4 0 0 0-4-4h-4a4 4 0 0 0-4 4v3a6.1 6.1 0 0 0 2 4.5" ></ path > < path d = "M20.97 5c0 2.1-1.6 3.8-3.5 4" ></ path > < path d = "M3 21c0-2.1 1.7-3.9 3.8-4" ></ path > < path d = "M6 13H2" ></ path > < path d = "M6.53 9C4.6 8.8 3 7.1 3 5" ></ path > < path d = "m8 2 1.88 1.88" ></ path > < path d = "M9 7.13v-1a3.003 3.003 0 1 1 6 0v1" ></ path > < / > } } pub const LUCIDE_BUG_PLAY : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;