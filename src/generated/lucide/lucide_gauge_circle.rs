use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.6 2.7a10 10 0 1 0 5.7 5.7" ></ path > < circle cx = "12" r = "2" cy = "12" ></ circle > < path d = "M13.4 10.6 19 5" ></ path > < / > } } pub const LucideGaugeCircle : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;