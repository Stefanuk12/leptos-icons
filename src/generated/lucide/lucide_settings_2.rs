use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 7h-9" ></ path > < path d = "M14 17H5" ></ path > < circle cy = "17" cx = "17" r = "3" ></ circle > < circle cx = "7" r = "3" cy = "7" ></ circle > < / > } } pub const LUCIDE_SETTINGS_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("width" , "24")] } ;