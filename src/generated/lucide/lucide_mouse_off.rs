use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6v.343" ></ path > < path d = "M18.218 18.218A7 7 0 0 1 5 15V9a7 7 0 0 1 .782-3.218" ></ path > < path d = "M19 13.343V9A7 7 0 0 0 8.56 2.902" ></ path > < path d = "M22 22 2 2" ></ path > < / > } } pub const LUCIDE_MOUSE_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor")] } ;