use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "12" r = "10" ></ circle > < circle cy = "12" cx = "12" r = "4" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;