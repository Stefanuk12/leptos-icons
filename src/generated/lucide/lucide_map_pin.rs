use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" ></ path > < circle r = "3" cx = "12" cy = "10" ></ circle > < / > } } pub const LUCIDE_MAP_PIN : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24")] } ;