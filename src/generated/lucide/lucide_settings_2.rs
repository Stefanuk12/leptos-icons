use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 7h-9" ></ path > < path d = "M14 17H5" ></ path > < circle r = "3" cy = "17" cx = "17" ></ circle > < circle cx = "7" cy = "7" r = "3" ></ circle > < / > } } pub const LUCIDE_SETTINGS_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;