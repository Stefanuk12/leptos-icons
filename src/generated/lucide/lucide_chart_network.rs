use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m13.11 7.664 1.78 2.672" ></ path > < path d = "m14.162 12.788-3.324 1.424" ></ path > < path d = "m20 4-6.06 1.515" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < circle r = "2" cy = "6" cx = "12" ></ circle > < circle cx = "16" r = "2" cy = "12" ></ circle > < circle cx = "9" cy = "15" r = "2" ></ circle > < / > } } pub const LUCIDE_CHART_NETWORK : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;