use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" x = "3" width = "18" y = "3" ></ rect > < circle cy = "12" cx = "12" r = "5" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_ALBUM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("width" , "24") , ("height" , "24")] } ;