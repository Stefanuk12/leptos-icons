use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" x = "3" width = "18" y = "3" ></ rect > < circle r = "5" cx = "12" cy = "12" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_ALBUM : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;