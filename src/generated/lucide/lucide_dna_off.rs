use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 2c-1.35 1.5-2.092 3-2.5 4.5L14 8" ></ path > < path d = "m17 6-2.891-2.891" ></ path > < path d = "M2 15c3.333-3 6.667-3 10-3" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "m20 9 .891.891" ></ path > < path d = "M22 9c-1.5 1.35-3 2.092-4.5 2.5l-1-1" ></ path > < path d = "M3.109 14.109 4 15" ></ path > < path d = "m6.5 12.5 1 1" ></ path > < path d = "m7 18 2.891 2.891" ></ path > < path d = "M9 22c1.35-1.5 2.092-3 2.5-4.5L10 16" ></ path > < / > } } pub const LUCIDE_DNA_OFF : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;