use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "8" ></ circle > < path d = "M12 2v7.5" ></ path > < path d = "m19 5-5.23 5.23" ></ path > < path d = "M22 12h-7.5" ></ path > < path d = "m19 19-5.23-5.23" ></ path > < path d = "M12 14.5V22" ></ path > < path d = "M10.23 13.77 5 19" ></ path > < path d = "M9.5 12H2" ></ path > < path d = "M10.23 10.23 5 5" ></ path > < circle cy = "12" r = "2.5" cx = "12" ></ circle > < / > } } pub const LUCIDE_SHIP_WHEEL : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;