use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 8.9V7H8l4 5-4 5h8v-1.9" ></ path > < / > } } pub const LucideSigmaSquare : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round")] } ;