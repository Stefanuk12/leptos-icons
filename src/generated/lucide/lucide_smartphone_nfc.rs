use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "1" width = "7" height = "12" x = "2" y = "6" ></ rect > < path d = "M13 8.32a7.43 7.43 0 0 1 0 7.36" ></ path > < path d = "M16.46 6.21a11.76 11.76 0 0 1 0 11.58" ></ path > < path d = "M19.91 4.1a15.91 15.91 0 0 1 .01 15.8" ></ path > < / > } } pub const LUCIDE_SMARTPHONE_NFC : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;