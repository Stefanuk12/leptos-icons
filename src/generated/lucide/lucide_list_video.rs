use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 12H3" ></ path > < path d = "M16 6H3" ></ path > < path d = "M12 18H3" ></ path > < path d = "m16 12 5 3-5 3v-6Z" ></ path > < / > } } pub const LUCIDE_LIST_VIDEO : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;