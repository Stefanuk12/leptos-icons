use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 8.32a7.43 7.43 0 0 1 0 7.36" ></ path > < path d = "M9.46 6.21a11.76 11.76 0 0 1 0 11.58" ></ path > < path d = "M12.91 4.1a15.91 15.91 0 0 1 .01 15.8" ></ path > < path d = "M16.37 2a20.16 20.16 0 0 1 0 20" ></ path > < / > } } pub const LUCIDE_NFC : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;