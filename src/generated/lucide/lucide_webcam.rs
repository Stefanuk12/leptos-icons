use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "10" r = "8" ></ circle > < circle cy = "10" r = "3" cx = "12" ></ circle > < path d = "M7 22h10" ></ path > < path d = "M12 22v-4" ></ path > < / > } } pub const LUCIDE_WEBCAM : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;