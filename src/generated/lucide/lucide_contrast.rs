use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "10" cx = "12" cy = "12" ></ circle > < path d = "M12 18a6 6 0 0 0 0-12v12z" ></ path > < / > } } pub const LUCIDE_CONTRAST : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;