use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "8" cy = "10" cx = "12" ></ circle > < circle r = "3" cx = "12" cy = "10" ></ circle > < path d = "M7 22h10" ></ path > < path d = "M12 22v-4" ></ path > < / > } } pub const LUCIDE_WEBCAM : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor")] } ;