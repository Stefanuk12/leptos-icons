use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" x = "3" y = "3" rx = "2" width = "18" ></ rect > < circle cx = "12" cy = "12" r = "5" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_ALBUM : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;