use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1" ></ path > < circle cx = "12" r = "2" cy = "8" ></ circle > < path d = "M12 10v12" ></ path > < path d = "M12 22c4.2 0 7-1.667 7-5-4.2 0-7 1.667-7 5Z" ></ path > < path d = "M12 22c-4.2 0-7-1.667-7-5 4.2 0 7 1.667 7 5Z" ></ path > < / > } } pub const LUCIDE_FLOWER_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("height" , "24")] } ;