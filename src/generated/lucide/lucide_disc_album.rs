use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" x = "3" rx = "2" width = "18" ></ rect > < circle r = "5" cx = "12" cy = "12" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_ALBUM : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;