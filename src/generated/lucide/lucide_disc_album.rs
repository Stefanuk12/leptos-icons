use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" x = "3" y = "3" rx = "2" ></ rect > < circle r = "5" cy = "12" cx = "12" ></ circle > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DISC_ALBUM : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;