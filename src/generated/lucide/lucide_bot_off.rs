use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13.67 8H18a2 2 0 0 1 2 2v4.33" ></ path > < path d = "M2 14h2" ></ path > < path d = "M20 14h2" ></ path > < path d = "M22 22 2 2" ></ path > < path d = "M8 8H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h12a2 2 0 0 0 1.414-.586" ></ path > < path d = "M9 13v2" ></ path > < path d = "M9.67 4H12v2.33" ></ path > < / > } } pub const LUCIDE_BOT_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;