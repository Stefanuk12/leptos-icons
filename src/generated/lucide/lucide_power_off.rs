use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18.36 6.64A9 9 0 0 1 20.77 15" ></ path > < path d = "M6.16 6.16a9 9 0 1 0 12.68 12.68" ></ path > < path d = "M12 2v4" ></ path > < path d = "m2 2 20 20" ></ path > < / > } } pub const LUCIDE_POWER_OFF : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;