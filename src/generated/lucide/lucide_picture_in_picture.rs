use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 4.5v5H3m-1-6 6 6m13 0v-3c0-1.16-.84-2-2-2h-7m-9 9v2c0 1.05.95 2 2 2h3" ></ path > < rect height = "7" width = "10" ry = "2" y = "13.5" x = "12" ></ rect > < / > } } pub const LUCIDE_PICTURE_IN_PICTURE : Path = Path { path : icon_path , props : & [("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round")] } ;