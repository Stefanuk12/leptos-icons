use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2c-1.35 1.5-2.092 3-2.5 4.5M9 22c1.35-1.5 2.092-3 2.5-4.5" ></ path > < path d = "M2 15c3.333-3 6.667-3 10-3m10-3c-1.5 1.35-3 2.092-4.5 2.5" ></ path > < path d = "m17 6-2.5-2.5" ></ path > < path d = "m14 8-1.5-1.5" ></ path > < path d = "m7 18 2.5 2.5" ></ path > < path d = "m3.5 14.5.5.5" ></ path > < path d = "m20 9 .5.5" ></ path > < path d = "m6.5 12.5 1 1" ></ path > < path d = "m16.5 10.5 1 1" ></ path > < path d = "m10 16 1.5 1.5" ></ path > < line x2 = "22" y1 = "2" y2 = "22" x1 = "2" ></ line > < / > } } pub const LUCIDE_DNA_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none")] } ;