use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 18-.722-3.25" ></ path > < path d = "M2 8a10.645 10.645 0 0 0 20 0" ></ path > < path d = "m20 15-1.726-2.05" ></ path > < path d = "m4 15 1.726-2.05" ></ path > < path d = "m9 18 .722-3.25" ></ path > < / > } } pub const LUCIDE_EYE_CLOSED : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round")] } ;