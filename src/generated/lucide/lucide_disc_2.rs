use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle cx = "12" cy = "12" r = "4" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;