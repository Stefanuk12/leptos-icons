use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20 7h-9" ></ path > < path d = "M14 17H5" ></ path > < circle cy = "17" r = "3" cx = "17" ></ circle > < circle r = "3" cy = "7" cx = "7" ></ circle > < / > } } pub const LUCIDE_SETTINGS_2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;