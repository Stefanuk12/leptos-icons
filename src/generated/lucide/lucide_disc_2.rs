use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "12" r = "10" cx = "12" ></ circle > < circle cy = "12" r = "4" cx = "12" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round")] } ;