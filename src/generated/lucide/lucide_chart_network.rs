use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m13.11 7.664 1.78 2.672" ></ path > < path d = "m14.162 12.788-3.324 1.424" ></ path > < path d = "m20 4-6.06 1.515" ></ path > < path d = "M3 3v16a2 2 0 0 0 2 2h16" ></ path > < circle r = "2" cx = "12" cy = "6" ></ circle > < circle cx = "16" cy = "12" r = "2" ></ circle > < circle r = "2" cy = "15" cx = "9" ></ circle > < / > } } pub const LUCIDE_CHART_NETWORK : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;