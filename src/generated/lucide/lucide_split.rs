use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 3h5v5" ></ path > < path d = "M8 3H3v5" ></ path > < path d = "M12 22v-8.3a4 4 0 0 0-1.172-2.872L3 3" ></ path > < path d = "m15 9 6-6" ></ path > < / > } } pub const LUCIDE_SPLIT : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;