use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2Z" ></ path > < path d = "M6 15v-2" ></ path > < path d = "M12 15V9" ></ path > < circle cx = "12" cy = "6" r = "3" ></ circle > < / > } } pub const LUCIDE_JOYSTICK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24")] } ;